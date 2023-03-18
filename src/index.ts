import { readFileSync } from "node:fs";

const lines = readFileSync("/home/jt/projects/rustbook/lines")
  .toString()
  .split("\n")
  .filter((_, idx) => idx % 2 == 0)
  .filter((_, idx) => idx >= 2 && idx < 4)
  .forEach((line) => console.log(line));

