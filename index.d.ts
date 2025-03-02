/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export declare function getMousePosition(): MousePosition
export declare function getPixelColorAtCursor(): PixelColor | null
export declare function getScreensInfo(): Array<ScreenInfo>
export declare class MousePosition {
  x: number
  y: number
}
export declare class PixelColor {
  r: number
  g: number
  b: number
  hex: string
}
export declare class ScreenInfo {
  x: number
  y: number
  width: number
  height: number
}
