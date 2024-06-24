import json

obj = json.loads(open("linestation.json", encoding="utf8").read())

out = []

for line in obj["businessObject"]:
    out.append({
        "name": line["lineName"],
        "data": [y for x in line["stations"] for y in (x["stationName"], 1)][:-1],
    })

open("map_init.json", "w", encoding="utf8").write(json.dumps(out, indent=2, ensure_ascii=False))
