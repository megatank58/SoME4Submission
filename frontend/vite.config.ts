import { defaultTheme  } from '@sveltepress/theme-default' 
import { sveltepress  } from '@sveltepress/vite'

import { defineConfig  } from 'vite'

const config  = defineConfig ({
  plugins : [
    sveltepress ({
      theme : defaultTheme (/** theme options */) 
    })
  ],
})

export default config 