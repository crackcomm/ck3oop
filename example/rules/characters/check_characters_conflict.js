const _ = require('lodash');
const utils = require('./../utils');

function checkCharacterConflict(
    context,
    {
        parser,
        gameFiles,
        enabledMods,
        enabledFiles,
    },
) {
    console.log("Checking game characters conflicts...")
    checkConflicts(context['game_characters']);

    console.log("Checking mod characters conflicts...")
    context['mods_characters'].forEach(mod => {
        console.log(`Checking ${mod.getMod().name} characters...`);
        checkConflicts(mod);
    });

    console.log("Checking play characters conflicts...")
    checkConflicts(context['play_characters']);

    function checkConflicts(characters){

        _.mapKeys(_.groupBy(characters, 'mother'), (characters2, mother) => {
            if (mother === "undefined") return;
            if (!characters[mother]){
                console.log(`Warning: mother ${mother} not found`);
            }
        });

        _.mapKeys(_.groupBy(characters, 'father'), (character, father) => {
            if (father === "undefined") return;
            if (!characters[father]){
                console.log(`Warning: father ${father} not found`);
            }
        });
    }
    return context;
}

checkCharacterConflict.name = "generateDynastyTree";
module.exports = checkCharacterConflict;
