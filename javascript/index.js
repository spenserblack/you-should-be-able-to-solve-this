#!/usr/bin/env node
const x = Infinity;

exports.x = x;

if (require.main === module) {
  console.log('x - 7 === 19 + x:', x - 7 === 19 + x);
}
