# bbnm-telebot
Powered by [teloxide](https://github.com/teloxide) 0.15.0.

Telegram bot for Six-Prison Alliance. Anti Bilibili b23 link tracking.

A toy project for learning, not guarantee of stable work.

### Usage

Replace `TELOXIDE_TOKEN` with your token from [@BotFather](https://telegram.me/BotFather)

```bash
docker build -t bbnm-telebot .
docker run -d --name bbnm-telebot --env-file .env bbnm-telebot
```

### Functions

- [ ] Bilibili Links
  - [x] Identify and parse Bilibili links
  - [x] bvid, aid, b23 link
  - [ ] Multi-link parse
  - [ ] Selectively replace b23 message content instead of deleting it all
- [ ] Video Download
  - [x] Get video info
  - [x] Set proper video caption
  - [ ] Download video (being throttled)

### Reference

Download video part: [bili-cli-rs](https://github.com/niuhuan/bili-cli-rs) .
