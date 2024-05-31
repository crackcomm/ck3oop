const fs = require("fs");
const path = require("path");

//**
// * Iterate over files in a directory.
// * @param {string} dirPath - Path to the directory
// * @param {function} fileCallback - Callback function to call for each file
// * @param {boolean} recursively - Whether to iterate over files recursively
// */
function iterateFiles(dirPath, fileCallback, recursively = false) {
  try {
    const files = fs.readdirSync(dirPath);
    files.forEach((file) => {
      const filePath = path.join(dirPath, file);
      const stats = fs.statSync(filePath);
      if (stats.isFile()) {
        fileCallback(dirPath, file, stats);
      } else if (recursively && stats.isDirectory()) {
        iterateFiles(filePath, fileCallback, recursively);
      }
    });
  } catch (err) {
      throw new Error(`Unable to iterate directory: ${dirPath}` + err);
  }
}

module.exports = {
  iterateFiles,
};
