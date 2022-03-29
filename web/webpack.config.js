const webpack = require('webpack');
const path = require('path');
const nodeExternals = require('webpack-node-externals');
const CopyWebpackPlugin = require('copy-webpack-plugin');

module.exports = {
  // モード値を production に設定すると最適化された状態で、
  // development に設定するとソースマップ有効でJSファイルが出力される
  mode: 'development',

  // メインとなるJavaScriptファイル（エントリーポイント）
  entry: './src/index.tsx',
  // ファイルの出力設定
  output: {
    //  出力ファイルのディレクトリ名
    path: `${__dirname}/dist/scripts`,
    // 出力ファイル名
    filename: 'main.js',
    publicPath: '/scripts/',
  },

  plugins: [
    // fix "process is not defined" error:
    // (do "npm install process" before running the build)
    new webpack.ProvidePlugin({
      process: 'process/browser',
    }),
    new CopyWebpackPlugin({
      patterns: [{ from: 'static', to: `${__dirname}/dist` }],
    }),
  ],

  externals: [/node_modules/],

  module: {
    rules: [
      {
        test: /\.jsx?$/,
        exclude: [/node_modules/],
        use: 'babel-loader',
      },
      {
        // 拡張子 .ts もしくは .tsx の場合
        test: /\.tsx?$/,
        exclude: /node_modules/,
        // TypeScript をコンパイルする
        use: 'ts-loader',
      },
    ],
  },
  // import 文で .ts や .tsx ファイルを解決するため
  resolve: {
    extensions: ['.ts', '.tsx', '.js', '.json'],
  },
  // ES5(IE11等)向けの指定（webpack 5以上で必要）
  target: ['web', 'es5'],

  devServer: {
    port: 3000,
    static: {
      directory: path.join(__dirname, 'static'),
      watch: true,
    },
    compress: true,
    open: false,
    hot: true,
    liveReload: true,
    historyApiFallback: true,
    webSocketServer: false // for escape error encoding
  },
};
