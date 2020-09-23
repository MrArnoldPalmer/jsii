#!/usr/bin/env npx ts-node

import * as fs from 'fs-extra';
import { join, resolve } from 'path';

const EMBEDDED_SOURCE = join(
  resolve(__dirname, '..', '..', 'runtime', 'webpack'), 'jsii-runtime.js'
);

const RT_DIR = resolve(__dirname, '..', 'assets');
const RT_PATH = resolve(RT_DIR, 'assets.go');

fs.removeSync(RT_PATH);

fs.open(RT_PATH, 'w', async (err, fd) => {
  if (err) {
    console.error(err);
    return;
  }

  await fs.write(fd, 'package assets\n\n');
  await fs.write(fd, 'var Tarball = []byte {');
  await fs.write(fd, getByteSlice(EMBEDDED_SOURCE).join(', '));
  await fs.write(fd, '}');
});

function getByteSlice(path: string) {
  const fileData = fs.readFileSync(path).toString('hex');
  const result = [];
  for (let i = 0; i < fileData.length; i += 2) {
    result.push(`0x${fileData[i]}${fileData[i+1]}`);
  }

  return result
}
