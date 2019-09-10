import { LiteralSource, renderTree, translateTypeScript } from "../../lib";
import { PythonVisitor } from "../../lib/languages/python";
import { visualizeTypeScriptAst } from "../../lib/util";

const DEBUG = true;

export async function ts2python(source: string): Promise<string> {
  const src = new LiteralSource(source, 'test.ts');

  if (DEBUG) {
    // tslint:disable-next-line:no-console
    console.log(await visualizeTypeScriptAst(src));
  }
  const result = await translateTypeScript(src, new PythonVisitor());

  return renderTree(result.tree);
}

export async function expectPython(source: string, expected: string) {
  await expect(stripEmptyLines(await ts2python(source))).toEqual(stripEmptyLines(stripCommonWhitespace(expected)));
}

function stripCommonWhitespace(x: string) {
  const lines = x.split('\n');
  const whitespaces = lines.filter(l => !emptyLine(l.trim())).map(l => l.match(/(\s*)/)![1].length);
  const minWS = Math.min(...whitespaces);
  return lines.map(l => l.substr(minWS)).join('\n');
}

function stripEmptyLines(x: string) {
  const lines = x.split('\n');
  while (lines.length > 0 && emptyLine(lines[0])) { lines.splice(0, 1); }
  while (lines.length > 0 && emptyLine(lines[lines.length - 1])) { lines.pop(); }
  return lines.join('\n');
}

function emptyLine(x: string) {
  return x.trim() === '';
}