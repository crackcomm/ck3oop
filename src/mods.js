const fs = require("fs");
const ini = require("ini");
const path = require("path");

//**
// * Mod class representing a mod file.
// * @param {string} filePath - Path to the mod file
// */
class Mod {

    constructor(filePath, fileName) {

        this.filePath = filePath;
        this.fileName = fileName;

        const data = fs.readFileSync(filePath, "utf-8");
        if (!data) {
            throw new Error("Error reading file");
        }
        const parsedData = ini.parse(data);

        this.version = parsedData.version || null;
        this.name = parsedData.name || null;
        this.picture = parsedData.picture || null;
        this.supported_version = parsedData.supported_version || null;
        this.path = parsedData.path || null;
        this.remote_file_id = parsedData.remote_file_id || null;
        this.resolvedPath = null;
    }
}

//**
// * Get the mod directory
// * @param {string} modDir - path to the mod directory
// * @param {string} modsDir - path to the mods directory
function resolveModDir(modPath, modsDir) {

    // If mod path is absolute it's probably in the
    // Program Files (x86)/Steam/steamapps/workshop/content/1158310 directory
    if (path.isAbsolute(modPath)) {
        return path.normalize(modPath);
    }

    // Otherwise it's probably in the
    // Documents/Paradox Interactive/Crusader Kings III/mods directory
    else {
        return path.join(modsDir, modPath);
    }
}

module.exports = {
    Mod,
    resolveModDir,
}