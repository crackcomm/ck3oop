import * as path from "node:path";
import {spawn, spawnSync} from "child_process";
import * as os from "node:os";
import {program} from "commander";

program.requiredOption("-t, --tauri-app-path <path>")
    .description("Path to the tauri app binary")

program.requiredOption("-d, --tauri-driver-path <path>")
    .description("Path to the tauri driver binary")

program.option("-w, --webdriver-path <path>")
    .description("Path to the webdriver binary")
program.command("run")
    .description("Run the e2e tests")
    .action(() => {
        console.log(program.opts())
    })

program.parse(process.argv);

function waitForMessage(process: any, message: any) {

    return new Promise((resolve: any, reject) => {
        process.stdout.on('data', (data: any) => {
            const output = data.toString();
            console.log(output);
            if (output.includes(message)) {
                resolve();
            }
        });

        process.stderr.on('data', (data: any) => {
            console.error(data.toString());
        });

        process.on('error', (err: any) => {
            reject(err);
        });

        process.on('exit', (code: any) => {
            if (code !== 0) {
                reject(new Error(`Process exited with code: ${code}`));
            }
        });
    });
}

let tauriDriver = spawn(
    program.opts()['tauriDriverPath'],
    ["--native-driver", program.opts()['webdriverPath'], "--native-port", "4566"],
    {
        stdio: [null, "pipe", "pipe"],
        detached: false
    }
)

process.on('exit', () => {
    tauriDriver.kill();
});

waitForMessage(tauriDriver, 'Microsoft Edge WebDriver was started successfully.')
    .then(() => {
        console.log('WebDriver started successfully.');
    })
    .catch((err) => {
        console.error('Failed to start WebDriver:', err);
        process.exit(1);
    });

// exit process after 5seconds
setTimeout(() => {
    console.log("Timeout");
    process.exit();
}, 5000);
