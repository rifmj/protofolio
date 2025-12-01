import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'protofolio',
  description: 'A Rust crate for generating AsyncAPI 3.0 specifications from Rust code annotations',
  
  base: '/protofolio/',
  
  // Internationalization configuration
  locales: {
    root: {
      label: 'English',
      lang: 'en',
      link: '/',
      title: 'protofolio',
      description: 'A Rust crate for generating AsyncAPI 3.0 specifications from Rust code annotations',
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
        editLink: {
          pattern: 'https://github.com/rifmj/protofolio/edit/main/docs/:path',
          text: 'Edit this page on GitHub'
        }
      }
    },
    ru: {
      label: 'Русский',
      lang: 'ru',
      link: '/ru/',
      title: 'protofolio',
      description: 'Rust библиотека для генерации спецификаций AsyncAPI 3.0 из аннотаций кода',
      themeConfig: {
        nav: [
          { text: 'Главная', link: '/ru/' },
          { text: 'Руководства', link: '/ru/guides/getting-started' },
          { text: 'Примеры', link: '/ru/examples/basic' },
          { text: 'Справочник', link: '/ru/reference/limitations' },
          { text: 'GitHub', link: 'https://github.com/rifmj/protofolio' }
        ],
        sidebar: {
          '/ru/guides/': [
            {
              text: 'Руководства',
              items: [
                { text: 'Начало работы', link: '/ru/guides/getting-started' },
                { text: 'Сообщения', link: '/ru/guides/messages' },
                { text: 'Операции', link: '/ru/guides/operations' },
                { text: 'Безопасность', link: '/ru/guides/security' },
                { text: 'Валидация', link: '/ru/guides/validation' },
                { text: 'Лучшие практики', link: '/ru/guides/best-practices' }
              ]
            }
          ],
          '/ru/examples/': [
            {
              text: 'Примеры',
              items: [
                { text: 'Базовые примеры', link: '/ru/examples/basic' },
                { text: 'Продвинутые примеры', link: '/ru/examples/advanced' },
                { text: 'Примеры интеграции', link: '/ru/examples/integration' }
              ]
            }
          ],
          '/ru/reference/': [
            {
              text: 'Справочник',
              items: [
                { text: 'Решение проблем', link: '/ru/reference/troubleshooting' },
                { text: 'Ограничения', link: '/ru/reference/limitations' },
                { text: 'Руководство по миграции', link: '/ru/reference/migration' },
                { text: 'Раскрытие макросов', link: '/ru/reference/macro-expansion' }
              ]
            }
          ]
        },
        editLink: {
          pattern: 'https://github.com/rifmj/protofolio/edit/main/docs/:path',
          text: 'Редактировать на GitHub'
        }
      }
    }
  },
  
  // Shared theme configuration
  themeConfig: {
    socialLinks: [
      { icon: 'github', link: 'https://github.com/rifmj/protofolio' }
    ],
    search: {
      provider: 'local'
    },
    footer: {
      message: 'Released under the Apache-2.0 and MIT licenses.',
      copyright: 'Copyright © 2024 protofolio contributors'
    }
  }
})
