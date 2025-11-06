# Testing AIDO Keybindings

## Quick Test

1. **Reload your shell configuration:**
   ```bash
   source ~/.zshrc
   ```

2. **Test Ctrl+O (DO mode):**
   - Press `Ctrl+O` in your terminal
   - You should see: "🤖 AIDO DO Mode - What do you want to do?"
   - Type: `list all files`
   - Press Enter
   - You should see the command generated and a confirmation prompt

3. **Test Ctrl+P (ASK mode):**
   - Press `Ctrl+P` in your terminal
   - You should see: "💬 AIDO ASK Mode - What's your question?"
   - Type: `what does ls do?`
   - Press Enter
   - You should see an answer from Claude

## If keybindings don't work:

1. **Make sure the shell integration is loaded:**
   ```bash
   tail -5 ~/.zshrc
   ```
   You should see:
   ```
   # AIDO keybindings - added automatically
   eval "$(aido setup-shell)"
   ```

2. **Manually reload:**
   ```bash
   eval "$(aido setup-shell)"
   ```

3. **Test that aido works from command line:**
   ```bash
   aido do "show current directory" -n
   ```

## Troubleshooting

**If vared doesn't work:**
The widget now uses `vared` for input, which is a built-in zsh function that properly handles line editing in widgets.

**If you're in tmux:**
Make sure tmux isn't intercepting the keybindings. You can test outside of tmux first.

**Check if keybindings are loaded:**
```bash
bindkey | grep aido
```

You should see:
```
"^O" aido-do-widget
"^P" aido-ask-widget
```

## Command Line Usage (Always Works)

If keybindings give you trouble, you can always use the command line:

```bash
aido do "your command here"
aido ask "your question here"
```

---

**Current Status:**
- ✅ Binary built and installed
- ✅ Shell integration code updated
- ✅ Keybindings configured (Ctrl+O and Ctrl+P)
- ⬜ You need to: `source ~/.zshrc` to activate
