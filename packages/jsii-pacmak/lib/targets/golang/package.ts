import { CodeMaker } from 'codemaker';
import { Assembly } from 'jsii-reflect';
import { join } from 'path';
import { EmitContext } from './emit-context';
import { ReadmeFile } from './readme-file';
import { Module, Submodule } from './module';

export class Package {
  public readonly packageName: string;
  public readonly rootModule: Module;

  public constructor(public readonly assembly: Assembly) {
    this.packageName = this.assembly.name
      .replace('@', '')
      .replace(/[^a-z0-9_.]/gi, '');
    this.rootModule = new Module(assembly);
  }

  public emit({ code }: EmitContext): void {
    if (this.assembly.readme?.markdown) {
      new ReadmeFile(this.packageName, this.assembly.readme.markdown);
    }

    const packageFile = `${join(...this.packageName.split('.'))}.go`;
    code.openFile(packageFile);
    code.line(`package ${this.packageName}`);
    code.line();
    this.rootModule.emit(code);
    code.closeFile(packageFile);

    this.emitSubmodules(code, this.rootModule);
  }

  public emitSubmodules(
    code: CodeMaker,
    parent: Module,
    parentPath = '',
  ): void {
    parent.submodules.forEach((submodule: Submodule) => {
      const nextPath = `${parentPath}/${submodule.name}`;
      const filename = `${nextPath}.go`;

      code.openFile(filename);
      code.line();
      submodule.emit(code);
      code.closeFile(filename);

      // recurse into next level
      this.emitSubmodules(code, submodule, nextPath);
    });
  }
}
