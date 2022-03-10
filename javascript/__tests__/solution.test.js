const x = require('../lib/x');

describe('solution', () => {
  test('x - 7 = 19 + x', () => {
    expect(x - 7).toEqual(19 + x);
  });
});
