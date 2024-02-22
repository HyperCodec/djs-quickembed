# quickembed
A simple npm package written in Rust to template discord.js embeds.

Uses the [APIEmbed](https://discord-api-types.dev/api/discord-api-types-v10/interface/APIEmbed) structure as a basis for the serialization model (some optional properties such as timestamp are ommitted).

### How to use
First, import the package (assuming you have installed it with `npm` already):
```js
// src/index.mjs
import * as QuickEmbed from "quickembed";
```

Next, you must create a TOML string with the embed template. This can be done by many means, but the best way of doing it is creating a simple TOML file and reading it:
```toml
# src/templates/hello.toml
title = "Hello, {% name %}!"
description = "This is a description."

[[field]]
name = "Field 1"
value = "This is a field."
```

```js
// src/index.mjs
const fs = require("node:fs");
const templateStr = fs.readFileSync("./src/templates/hello.toml", "utf-8");
```

And now you can use `QuickEmbed.parse_template` to initialize a template:
```js
// src/index.mjs
const helloTemplate = QuickEmbed.parse_template(templateStr);
```

Finally, you render your embed with `QuickEmbed.render` and send it:
```js
// src/index.mjs
let helloEmbed = QuickEmbed.render(helloTemplate, { name: "World" });
channel.send({ embeds: [helloEmbed] });
```