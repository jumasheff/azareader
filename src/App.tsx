import { CSSProperties, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const superInfo = "https://azattyk.org";
  const [pageSource, setPageSource] = useState("<p>Greetings!</p>");
  const [isStart, setIsStart] = useState(false);

  async function sendRequestToBackend(URL: string) {
    const msg: string = await invoke("greet", { name: URL });
    setPageSource(msg);
  }

  useEffect(() => {
    if (isStart) {
      sendRequestToBackend(superInfo);
    }
  }, [isStart]);

  useEffect(() => {
    function handleMessage(event: MessageEvent) {
      if (event.data.type === "linkClicked") {
        let urlObj = new URL(event.data.href);
        if (urlObj.origin.includes("localhost")) {
          urlObj = new URL(superInfo + urlObj.pathname);
        }
        sendRequestToBackend(urlObj.href);
      } else {
        console.log(event.data);
      }
    }

    window.addEventListener("message", handleMessage);
    return () => window.removeEventListener("message", handleMessage);
  }, []);

  const style = {
    margin: "0px",
    padding: "0px",
    overflow: "hidden",
  };

  const iFrameStyle: CSSProperties = {
    overflow: "hidden",
    overflowX: "hidden",
    overflowY: "hidden",
    height: "100%",
    width: "100%",
    position: "absolute",
    top: "0px",
    left: "0px",
    right: "0px",
    bottom: "0px",
  };

  const script = `
    <script>
      const links = document.querySelectorAll("a");
      links.forEach((link) => {
        link.addEventListener("click", (event) => {
          event.preventDefault();
          window.parent.postMessage(
            {
              type: "linkClicked",
              href: link.href,
            },
            "*"
          );
        });
      });
    </script>
  `;

  const srcDoc = pageSource.replace("</body>", script + "</body>");

  if (!isStart) {
    return <button onClick={() => setIsStart(true)}>Start</button>;
  }

  return (
    <div style={style}>
      <iframe
        srcDoc={srcDoc}
        style={iFrameStyle}
        title="greet"
        width="100%"
        height="100vh"
      />
    </div>
  );
}

export default App;
