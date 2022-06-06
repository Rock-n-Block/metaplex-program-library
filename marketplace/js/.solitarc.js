// @ts-check
const path = require('path');
const programDir = path.join(__dirname, '..', 'program');
const idlDir = path.join(__dirname, 'idl');
const sdkDir = path.join(__dirname, 'src', 'generated');
const binaryInstallDir = path.join(__dirname, '.crates');

module.exports = {
  idlGenerator: 'anchor',
  programName: 'marketplace',
  programId: '81Xv3QwiLvcWgrMXKkhPRWrYsHrdTCbBTd3N7W4rHt8H',
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};
