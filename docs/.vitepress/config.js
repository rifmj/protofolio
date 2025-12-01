import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'protofolio',
  description: 'A Rust crate for generating AsyncAPI 3.0 specifications from Rust code annotations',
  
  // Important: Set base to your repository name for GitHub Pages
  base: '/protofolio/',
  
  // Theme configuration
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guides', link: '/guides/getting-started' },
      { text: 'Examples', link: '/examples/basic' },
      { text: 'Reference', link: '/reference/limitations' },
      { text: 'GitHub', link: 'https://github.com/rifmj/protofolio' }
    ],
    
    sidebar: {
      '/guides/': [
        {
          text: 'Guides',
          items: [
            { text: 'Getting Started', link: '/guides/getting-started' },
            { text: 'Messages', link: '/guides/messages' },
            { text: 'Operations', link: '/guides/operations' },
            { text: 'Security', link: '/guides/security' },
            { text: 'Validation', link: '/guides/validation' },
            { text: 'Best Practices', link: '/guides/best-practices' }
          ]
        }
      ],
      '/examples/': [
        {
          text: 'Examples',
          items: [
            { text: 'Basic Examples', link: '/examples/basic' },
            { text: 'Advanced Examples', link: '/examples/advanced' },
            { text: 'Integration Examples', link: '/examples/integration' }
          ]
        }
      ],
      '/reference/': [
        {
          text: 'Reference',
          items: [
            { text: 'Troubleshooting', link: '/reference/troubleshooting' },
            { text: 'Limitations', link: '/reference/limitations' },
            { text: 'Migration Guide', link: '/reference/migration' },
            { text: 'Macro Expansion', link: '/reference/macro-expansion' }
          ]
        }
      ]
    },
    
    socialLinks: [
      { icon: 'github', link: 'https://github.com/rifmj/protofolio' }
    ],
    
    search: {
      provider: 'local'
    },
    
    footer: {
      message: 'Released under the Apache-2.0 and MIT licenses.',
      copyright: 'Copyright © 2024 protofolio contributors'
    },
    
    editLink: {
      pattern: 'https://github.com/rifmj/protofolio/edit/main/docs/:path',
      text: 'Edit this page on GitHub'
    }
  }
})

