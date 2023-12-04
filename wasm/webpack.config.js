// Used to copy files in the dist folder:
const CopyWebpackPlugin = require("copy-webpack-plugin");
// Used to run wasm-pack from the webpack command before bundeling:
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const path = require("path");
const toml = require("toml");
require("toml-require").install({ toml: toml });
const cargo = require("./Cargo.toml");

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(["index.html","assets/"]),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "."),
      // This ensures that it has the same name as with running "wasm-pack build":
      outName: cargo.package.name.replace(/-/g, "_"),
      // forceMode: "release",
      forceMode: "development"
    }),
  ],
  // Webassembly is a experimental feature and has to be
  // manually enabled:
  experiments: {
    asyncWebAssembly: true,
    syncWebAssembly: true,
  },
};
