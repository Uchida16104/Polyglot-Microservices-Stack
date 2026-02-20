import "../styles/globals.css";
import Script from "next/script";

export default function App({ Component, pageProps }) {
  return (
    <>
      <Script
        src="https://unpkg.com/htmx.org@1.9.12"
        strategy="beforeInteractive"
      />
      <Script
        src="https://unpkg.com/hyperscript.org@0.9.13"
        strategy="beforeInteractive"
      />
      <Script
        src="https://cdn.jsdelivr.net/npm/alpinejs@3.14.1/dist/cdn.min.js"
        strategy="afterInteractive"
      />
      <Component {...pageProps} />
    </>
  );
}
