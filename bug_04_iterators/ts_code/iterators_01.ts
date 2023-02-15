function get_strings() : string[] {
  return [
    "EEE", "DDD", "", "AAA", "CCC", "", "BBB", "FFF"
  ];
}

let make_lowercase = (s: string) : string => {
  return s.toLowerCase();
}

let vs = get_strings();
console.log("Unsorted strings:");
for (let s of vs) {
  console.log(s)
}
console.log("One more time to make sure all the bits are in the right places:", vs)

console.log("Sorted non-empty strings")
vs = vs.filter( str => str.length > 0 )
           .sort();
for (let s of vs) {
  console.log(s)
}

console.log("Lowercased sorted non-empty strings")
vs.forEach((s, index) => {
  vs[index] = make_lowercase(s)
});

console.log(vs);

