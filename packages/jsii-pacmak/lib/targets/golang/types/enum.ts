import { CodeMaker } from 'codemaker';
import { EnumType } from 'jsii-reflect';
import { GoType } from './go-type';
import { Module } from '../module';

// TODO: This whole thing
export class Enum extends GoType {
  public constructor(parent: Module, public type: EnumType) {
    super(parent, type);
  }

  public emit(code: CodeMaker) {
    code.line(`// enum ${this.name}`);
  }

  public get dependencies(): Module[] {
    return [];
  }
}
