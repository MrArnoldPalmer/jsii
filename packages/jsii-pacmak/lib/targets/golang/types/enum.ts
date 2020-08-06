import { CodeMaker } from 'codemaker';
import { EnumType } from 'jsii-reflect';
import { GoType } from './go-type';
import { Module } from '../module';

export class Enum extends GoType {
  public constructor(parent: Module, public type: EnumType) {
    super(parent, type);
  }

  public emit(code: CodeMaker) {
    code.line(`// enum ${this.name}`);
  }
}
