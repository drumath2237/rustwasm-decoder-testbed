import "./style.css";

import {
  decode_webp,
  deserialize_metajson,
  test_func,
  zip_test_wasm,
} from "./wasm/sog-wasm";

import sogPath from "../../sample_data/pizza.sog?url";
import metaJsonPath from "../../sample_data/pizza/meta.json?url";
import webpPath from "../../sample_data/pizza/means_l.webp?url";

function main() {
  fetch(sogPath).then((res) => res.arrayBuffer()).then((data) => {
    const sogData = new Uint8Array(data);
    const files = zip_test_wasm(sogData).map((m) => {
      return { name: m.name, data: m.data };
    });
    console.log(files);
  });

  fetch(metaJsonPath).then((res) => res.text()).then((json) => {
    const metaData = deserialize_metajson(json);
    console.log(metaData);
  });

  fetch(webpPath).then((res) => res.arrayBuffer()).then((buf) => {
    const imageData = new Uint8Array(buf);
    const pixels = decode_webp(imageData);
    console.log(pixels);
  });

  console.log(test_func());
}

main();
