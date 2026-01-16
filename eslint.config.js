import globals from "globals";
import pluginVue from "eslint-plugin-vue";

export default [
  // 1. 指定检查的文件类型
  {
    files: ["**/*.{js,mjs,cjs,vue}"],
  },
  // 2. 配置全局变量 (解决 'window is not defined' 等报错)
  {
    languageOptions: { 
      globals: {
        ...globals.browser, 
        ...globals.node 
      } 
    }
  },
  // 3. 引入 Vue 的推荐配置
  ...pluginVue.configs["flat/recommended"],
  
  // 4. 自定义规则
  {
    rules: {
      // 在这里覆盖你不喜欢的规则，例如：
      // "vue/multi-word-component-names": "off" 
    }
  }
];