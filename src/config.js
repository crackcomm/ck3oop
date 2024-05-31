const path = require("path");
const os = require("os");

//**
// * Config class representing `ck3oop.json` configuration file.
// * @param {Object} obj - Configuration object
// * @param {string} obj.modDir - Path to the mod directory
// * @param {string} obj.rulesDir - Path to the rules directory
// */
class Config {

    constructor(obj) {
        this.gameDir = obj.gameDir;

        if (this.gameDir === undefined) {
            throw new Error("gameDir not found in config file");
        }

        this.modDir = obj.modDir || getDefaultModDir();
        this.rulesDir = obj.rulesDir || path.join(process.cwd(), "rules");

        if (this.modDir.startsWith(".")) {
            this.modDir = path.join(process.cwd(), this.modDir);
        }

        if (this.rulesDir.startsWith(".")) {
            this.rulesDir = path.join(process.cwd(), this.rulesDir);
        }

        if (this.gameDir.startsWith(".")) {
            this.gameDir = path.join(process.cwd(), this._gameDir);
        }

        this.rules = obj.rules || [];
    }

    getGameDir() {
        return path.join(this.gameDir, "game").normalize();
    }
}

//**
// * Get the default mod directory
// * @returns {string} - Default mod directory
// */
function getDefaultModDir() {
    return path.join(...[
        os.homedir(),
        "Documents",
        "Paradox Interactive",
        "Crusader Kings III",
    ])
}

module.exports = {
    Config,
    getDefaultModDir,
}
