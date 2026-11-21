import { expect, test } from "bun:test";
import { ready } from "./index";
test("ready", () => expect(ready).toBe(true));
