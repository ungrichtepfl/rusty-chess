// Used to copy files in the dist folder:
const CopyWebpackPlugin = require("copy-webpack-plugin");
// Used to run wasm-pack from the webpack command before bundeling:
const path = require("path");

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "production",
  plugins: [new CopyWebpackPlugin(["index.html", "assets/"])],
  // Webassembly is a experimental feature and has to be
  // manually enabled:
  experiments: {
    asyncWebAssembly: true,
    syncWebAssembly: true,
  },
};
