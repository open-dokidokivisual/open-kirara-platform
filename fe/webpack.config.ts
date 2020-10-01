import path from 'path'
import { Configuration } from 'webpack'

const config: Configuration = {
  mode: 'development',
  entry: {
    main: './src/main.ts'
  },
  output: {
    path: path.join(__dirname, 'dist'),
    filename: '[name].js'
  },
  module: {
    rules: [
      {
        test: /.ts$/,
        use: 'ts-loader',
        exclude: '/node_modules/'
      }
    ]
  },
  resolve: {
    extensions: ['.ts', '.js'],
    alias: {
      vue: 'vue/dist/vue.js'
    }
  }
};

export default config;
