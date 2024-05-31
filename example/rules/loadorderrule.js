const path = require("path");
const fs = require("fs");
const _ = require('lodash');
const utils = require('./utils');

function loadOrderRule(
    context,
    {
        parser,
        gameFiles,
        enabledMods,
        enabledFiles,
    },
) {

    // Helper functions
    const util = utils.ctx(context);

    // Read the load order rules
    const loadOrderRulesPath = util.buildInPath('loadOrder.json');
    const loadOrderRules = util.readJsonFile(loadOrderRulesPath);

    // Check if any of the rules allow expansion
    loadOrderRules['rules'].forEach((rule) => {
        if (typeof rule.expand !== 'undefined') {
            if (rule.expand.how === "any") {
                equalityLoadOrderByName = utils.powerSet(rule.mods);
            }
        }
    });

    // Write the expanded load order rules to a file
    const equalityLoadOrderByNamePath = util.buildOutPath('equalityLoadOrderByName.json');
    util.writeJsonFile(equalityLoadOrderByNamePath, equalityLoadOrderByName);

    // Check for conflicting files
    const conflictingFiles = {};
    for (let [filePath, data] of Object.entries(enabledFiles)) {

        // only one mod overwrites the file
        if (data.length < 2) {
            continue;
        }

        // otherwise allow overriding by predefined rules
        let fileModNames = data.map((x) => x.modName);

        if (!equalityLoadOrderByName.some((rule)=> _.isEqual(rule, fileModNames))) {
            conflictingFiles[filePath] = data;
        }
    }

    // Save conflicting files to a file
    util.writeJsonFile(util.buildOutPath('conflictingFiles.json'), conflictingFiles);
    return context;
}

loadOrderRule.name = "loadOrderRule";
module.exports = loadOrderRule;


