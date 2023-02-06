import * as React from "react";
import Svg, { SvgProps, Rect, Path } from "react-native-svg";
const SvgStrLvl90 = (props: SvgProps) => (
  <Svg
    xmlns="http://www.w3.org/2000/svg"
    width={'100%'}
    height={'100%'}
    viewBox="0 0 1080 1080"
    xmlSpace="preserve"
    {...props}
  >
    <Rect width="100%" height="100%" fill="transparent" />
    <Path
      style={{
        stroke: "#0008ff",
        strokeWidth: 3,
        strokeDasharray: "none",
        strokeLinecap: "butt",
        strokeDashoffset: 0,
        strokeLinejoin: "miter",
        strokeMiterlimit: 4,
        fill: "#000",
        fillRule: "nonzero",
        opacity: 1,
      }}
      vectorEffect="non-scaling-stroke"
      transform="translate(285 285)"
      d="m329.8 235.69 62.83-82.71 42.86 32.56-62.83 82.75zm-12.86-9.53 66.81-88-45-34.15-66.81 88zm-27.48-97.78-19.3 39.57 57-75-42.51-32.3-36.24 47.71zm-20.74-73.24-46.64-35.43-42 55.31 53.67 26.17zm107 235.52-139-102.71-9.92.91 4.56 2 62.16 138.43-16.52 2.25-57.68-128.5-40-17.7-4-30.84 39.41 19.42 36.36-3.33 17-34.83-110.9-54.09-80.68 112.51L177.6 346.67l-22.7 145.62H341V372.62l35.29-48.93L387 275.77z"
      strokeLinecap="round"
    />
  </Svg>
);
export default SvgStrLvl90;
