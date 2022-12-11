import { useEffect, useRef } from 'react';
import 'xterm/css/xterm.css';
import { Terminal } from 'xterm';
import { FitAddon } from 'xterm-addon-fit';
import { WebLinksAddon } from 'xterm-addon-web-links';
import { Unicode11Addon } from 'xterm-addon-unicode11';
import { SerializeAddon } from 'xterm-addon-serialize';
import { AttachAddon } from 'xterm-addon-attach';
import { SearchAddon } from 'xterm-addon-search';
// import { WebglAddon } from 'xterm-addon-webgl';

interface TerminalProps {
  api: string;
  params: string;
}

export const WebTerminal = (props: TerminalProps) => {
  const tty = useRef<HTMLDivElement>(null);
  let terminal: Terminal;

  const fitAddon = new FitAddon();

  useEffect(() => {
    if (!terminal) {
      const url = 'ws://127.0.0.1:9099/apps/terminal/openpty';

      const ws = new WebSocket(url);

      const setTerminalSize = (cols: number, rows: number) => {
        const size = JSON.stringify({ cols: cols, rows: rows + 1 });
        ws.send(new TextEncoder().encode('\x01' + size));
      };
      console.log('new terminal');
      terminal = new Terminal({
        fontSize: 14,
        fontFamily: 'Ubuntu Mono, courier-new, courier, monospace',
        cursorBlink: true,
        convertEol: true,
        // screenReaderMode: true,
        allowProposedApi: true,
        theme: {
          background: '#fdf6e3',
          foreground: '#3b3b3b',
          cursor: 'gray',
          cursorAccent: '#5b5b5b',
          // selectionBackground: 'rgb(168, 174, 167)'
        },
      });

      console.log(terminal);

      setInterval(() => {
        console.log(terminal.options.theme);
      }, 2_000);

      terminal.open(tty.current!);

      terminal.loadAddon(fitAddon);
      terminal.loadAddon(new WebLinksAddon());
      terminal.loadAddon(new Unicode11Addon());
      terminal.loadAddon(new SerializeAddon());
      terminal.loadAddon(new SearchAddon());
      // terminal.loadAddon(new WebglAddon());

      ws.onopen = () => {
        terminal.loadAddon(new AttachAddon(ws));
        terminal.focus();
        setTerminalSize(terminal.cols, terminal.rows);
        terminal.onResize(({ cols, rows }) => {
          setTerminalSize(cols, rows);
        });
        setTimeout(() => fitAddon.fit());
      };

      // terminal.attachCustomKeyEventHandler((event) => {
      //     if (event.ctrlKey && event.code === 'KeyC' && event.type === 'keydown') {
      //         navigator.clipboard.writeText(terminal.getSelection()).then((value) => console.log(value));
      //         return false;
      //     }
      //     return true;
      // })

      window.onresize = () => fitAddon.fit();

      // setTimeout(() => fitAddon.fit());
    }

    return () => {
    };
  }, [props.api, props.params]);

  return (
    <div ref={tty} style={{
      height: '100%',
      width: '100%',
    }}></div>
  );
};
