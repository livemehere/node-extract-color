import test from 'ava'

import { getMousePosition, getPixelColorAtCursor } from '../index.js'

test('getMousePosition from native', (t) => {
  t.is(typeof getMousePosition().x, 'number')
  t.is(typeof getMousePosition().y, 'number')
})

test('getPixelColorAtCursor from native', (t) => {
  const color = getPixelColorAtCursor();
  t.is(typeof color.r, 'number')
  t.is(typeof color.g, 'number')
  t.is(typeof color.b, 'number')
})
