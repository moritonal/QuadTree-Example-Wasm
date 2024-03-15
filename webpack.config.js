const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CssMinimizerPlugin = require("css-minimizer-webpack-plugin");
const TerserPlugin = require("terser-webpack-plugin");
const HtmlWebpackPlugin = require('html-webpack-plugin');

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    index: "./js/index.tsx"
  },
  resolve: {
    extensions: ['.js', '.jsx', '.ts', '.tsx'],
  },
  performance: {
    hints: false,
    maxEntrypointSize: 512000,
    maxAssetSize: 512000
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
      {
        test: /\.css$/i,
        use: ["style-loader", "css-loader"],
      },
      {
        test: /\.wasm$/,
        type: "webassembly/async", // Specify module type for WebAssembly files
      },
    ],
  },
  output: {
    path: dist,
    filename: "[name].js"
  },
  devServer: {
    static: {
      directory: dist,
    },
  },
  optimization: {
    minimize: true,
    mangleWasmImports: true,
    minimizer: [
      new CssMinimizerPlugin(),
      new TerserPlugin()
    ],
    splitChunks: {
      cacheGroups: {
        commons: {
          test: /[\\/]node_modules[\\/]/,
          // cacheGroupKey here is `commons` as the key of the cacheGroup
          name(module, chunks, cacheGroupKey) {

            const moduleFileName = module
              .identifier()
              .split('/')
              .reduceRight((item) => item);

            // remove js from end of moduleFileName
            const moduleFileNameWithoutExtension = moduleFileName.substring(0, moduleFileName.length - 3);

            const allChunksNames = chunks.map((item) => item.name).join('~');
            return `${cacheGroupKey}-${allChunksNames}-${moduleFileNameWithoutExtension}`;
          },
          chunks: 'all',
        },
      },
    },
  },
  experiments: {
    asyncWebAssembly: true, // Enable WebAssembly experiments
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
    new HtmlWebpackPlugin({
      templateContent: `
        <html>
          <body>
            <div id="root"></div>
          </body>
        </html>
      `})
  ]
};
