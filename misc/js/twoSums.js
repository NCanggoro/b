const twoSums = (arr, target) => {
	let set = [ ... new Set(arr)]
	let temp = {};
	let ans = [];

	for(let i in set) {
		let add = target - set[i];
		if(add in temp) {
			ans.push([add, set[i]]);
		}
		
		temp[set[i]] = i
	}


	if (ans.length > 0) {
		return ans[0];
	}
	return [];
}


let arr = [2, 7, 6, 7, 3, 5, 4, 11, 15]

console.log(twoSums(arr, 9));