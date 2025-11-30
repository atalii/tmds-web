import { parse_byte, State } from "../../alg/pkg";

let st = State.new()

function runEncode(_: Event) {
	const data = document.querySelector("#data-d")
	if (!(data instanceof HTMLInputElement)) {
		return;
	}

	try {
		const value = parse_byte(data.value);
		document.querySelector("div.input-bytes-info")!.classList.remove("emph");
		const converted = st.convert(value);

		document.querySelector("#contenthead")!.insertAdjacentHTML("afterend", `
		  <tr>
		    <td class="input"><code>${(value >>> 0).toString(2).padStart(9, '0')}</code></td>
		    <td><input type="checkbox" ${converted.one_dominated ? "checked" : ""} disabled/></td>
		    <td class="output"><code>${(converted.repr >>> 0).toString(2).padStart(10, '0')}</code></td>
		    <td class="bias"><code>${st.cnt}</code></td>
		  </tr>
	        `)
	} catch (e) {
		console.error(e);
		document.querySelector("div.input-bytes-info")!.classList.add("emph");
	}
}

function runClick(_: Event) {
	st.rst()

	document.querySelector("#contenthead")!.insertAdjacentHTML("afterend", `
	  <tr>
	    <td colspan="4" class="bias"><code>${st.cnt}</code></td>
	  </tr>
	`)
}

document.querySelector("#input-bytes")!.addEventListener("submit", runEncode)
document.querySelector("#rst")!.addEventListener("click", runClick)
