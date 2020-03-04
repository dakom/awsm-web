const { resolve } = require('path');

module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'node',
  globals: {
    'ts-jest': {
      tsConfig: 'tsconfig.json'
    }
  },
  moduleNameMapper: {
    '^@components/(.*)$': resolve(__dirname, './typescript/components/$1'),
    '^@utils/(.*)$': resolve(__dirname, './typescript/utils/$1'),
    '^@settings/(.*)$': resolve(__dirname, './typescript/settings/$1'),
    '^@styles/(.*)$': resolve(__dirname, './typescript/styles/$1'),
    '^@events/(.*)$': resolve(__dirname, './typescript/events/$1')
  },
};