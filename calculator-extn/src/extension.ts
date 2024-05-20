import * as vscode from 'vscode';
import { calculator, Types } from './calculator';
import { WasmContext, Memory } from '@vscode/wasm-component-model';

export async function activate(context: vscode.ExtensionContext) {

    const filename = vscode.Uri.joinPath(context.extensionUri, 'calculator_impl.wasm');
    const bits = await vscode.workspace.fs.readFile(filename);
    const module = await WebAssembly.compile(bits);

    const wasmContext: WasmContext.Default = new WasmContext.Default();
    const instance = await WebAssembly.instantiate(module, calculator._.imports.create({}, wasmContext));
    wasmContext.initialize(new Memory.Default(instance.exports));
    
    const api = calculator._.exports.bind(
        instance.exports as calculator._.Exports,
        wasmContext
    );

    let disposable = vscode.commands.registerCommand('calculator-extn.helloWorld', () => {
		const calculator = new api.types.Engine();
		calculator.pushOperand(1);
		calculator.pushOperand(2);
		calculator.pushOperation(Types.Operation.add);
		const result = calculator.execute();

		vscode.window.showInformationMessage(`1 + 2 = ${result}`);
    });

    context.subscriptions.push(disposable);
}

export function deactivate() {}
