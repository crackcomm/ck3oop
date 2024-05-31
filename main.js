const fs = require("fs");
const path = require("path");
const mods = require("./src/mods");
const files = require("./src/files");
const config = require("./src/config");
const {resolveModDir} = require("./src/mods");
const jomini = require("jomini");

// working directory as first argument
if (process.argv.length > 2) {
    process.chdir(process.argv[2]);
}

// set the working directory
const workDir = process.cwd();
console.log(`Working in directory: ${workDir}`)

// set the config file
const configFile = path.join(workDir, 'ck3oop.json');
console.log(`Using config file: ${configFile}`)

// if the config file is not found, throw error
if (!fs.existsSync(configFile)) {
    throw new Error(`Config file not found: ${configFile}`);
}

// load the config file
const CFG = new config.Config(JSON.parse(fs.readFileSync(configFile, 'utf-8')));
console.log("Mod directory:", CFG.modDir);

// read dlc_load.json
console.log("Reading dlc_load.json...")
const dlcLoadFile = path.join(workDir, "dlc_load.json");
const DLC_LOAD = JSON.parse(fs.readFileSync(dlcLoadFile, 'utf-8'));
console.log(DLC_LOAD['enabled_mods']);


// build the mod list
console.log("Creating enabled mods...")
let ENABLED_MODS = [];
DLC_LOAD['enabled_mods'].forEach(fileName => {
    let mod = new mods.Mod(path.join(CFG.modDir, fileName), fileName);
    mod.resolvedPath = resolveModDir(mod.path, CFG.modDir);
    ENABLED_MODS.push(mod);
});
console.log(ENABLED_MODS.map(mod => mod.name));

// read enabled rules
console.log("Reading enabled rules...")
let ENABLED_RULES = [];
CFG.rules = CFG.rules.map(rule => {
    let rulePath = path.join(CFG.rulesDir, rule);
    if (!fs.existsSync(rulePath)) {
        throw new Error(`Rule file not found: ${rulePath}`);
    }
    let ruleFunc = require(rulePath);
    ENABLED_RULES.push(ruleFunc);
});
console.log(ENABLED_RULES.map(rule => rule.name));

// read all files of the enabled_mods
ENABLED_MODS.forEach(mod => {
    console.log(`Reading files of mod: ${mod.name}`);
    mod['files'] = [];
    files.iterateFiles(mod.resolvedPath, function (dirPath, filePath, stats) {
        let dir = dirPath.replace(mod.resolvedPath, "");
        let file = {
            "fullPath": path.join(mod.resolvedPath, dir, filePath),
            "path": path.join(dir, filePath),
            "dir": dir,
            "file": filePath,
            "stats": stats
        };
        mod.files.push(file);
    }, true);
});

let BASE_GAME_FILES = [];
// read all files of the base game
if (CFG.getGameDir() !== null) {
    console.log(`Reading files of the base game...`)
    files.iterateFiles(CFG.getGameDir(), function (dirPath, filePath, stats) {
        let dir = dirPath.replace(CFG.getGameDir(), "");
        BASE_GAME_FILES.push({
            "fullPath": path.join(CFG.getGameDir(), dir, filePath),
            "path": path.join(dir, filePath),
            "file": filePath,
            "dir": dir,
            "stats": stats
        });
    }, true);
}

// create files load resolution order
console.log("Creating load resolution order...")
let ENABLED_FILES = {};

// mod files
ENABLED_MODS.forEach(mod => {
    mod.files.forEach(file => {
        if (!ENABLED_FILES[file.path]) {
            ENABLED_FILES[file.path] = [
                {
                    "modName": mod.name,
                    "modPath": mod.path,
                    "fullPath": path.join(mod.resolvedPath, file.path),
                }
            ];
        } else {
            ENABLED_FILES[file.path].push(
                {
                    "modName": mod.name,
                    "modPath": mod.path,
                    "fullPath": path.join(mod.resolvedPath, file.path),
                }
            );
        }
    });
});

// declare the context
let CONTEXT = {
    "workDir": workDir,
    "config": CFG,
    "rules": ENABLED_RULES.map(rule => rule.name),
};

// function used to build args for each rule
const runRule = (rule, parser) => {
    return rule(CONTEXT, {
        "parser": parser,
        "gameFiles": BASE_GAME_FILES,
        "enabledMods": ENABLED_MODS,
        "enabledFiles": ENABLED_FILES,
    });
}

// run the rules
jomini.Jomini.initialize().then((parser) => {

    console.log("Running rules...")
    ENABLED_RULES.forEach(rule => {
        console.log(`Running rule: ${rule.name}`);
        CONTEXT = runRule(rule, parser);
        if (!CONTEXT) {
            throw new Error(`Rule ${rule.name} did not return a context.`);
        }
    });
});
