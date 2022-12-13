const fs = require('fs');
const path = require('path');
function partA() {
    // let p = path.resolve(__dirname, "../../data/day13/test.txt");
    let p = path.resolve(__dirname, "../../data/day13/real.txt");
    let input = fs.readFileSync(p);
    input = String(input).split("\n\n");

    let sum = 0;
    for (let i = 0; i < input.length; i++) {
        input[i] = input[i].split('\n');
        let pair1 = eval(input[i][0]);
        let pair2 = eval(input[i][1]);
        console.log("ENTER COMPARE FUNC");
        console.log(pair1);
        console.log(pair2);
        if (compare(pair1, pair2) == true) {
            console.log(i+1+ ": RESULT TRUE");
            sum += i + 1;
        }
        console.log("\n\n");
    }
    console.log("sum: " + sum);
}

function partB() {
    // let p = path.resolve(__dirname, "../../data/day13/test.txt");
    let p = path.resolve(__dirname, "../../data/day13/real.txt");
    let input = fs.readFileSync(p);
    input = String(input).split("\n\n");
    input = input.map((pair) => pair.split("\n")).flat();
    input[input.length - 1] = "[[2]]"; //replace blank line
    input.push("[[6]]");
    input.forEach((el) => console.log(el))
    input = input.sort((a, b) => compare(eval(b), eval(a)));
    let ans = (input.indexOf("[[2]]") + 1) * (input.indexOf("[[6]]") + 1);
    console.log(input);
    console.log("sum: " + ans);
}


function compare(pair1, pair2, depth=0) {
    console.log(pair1, pair2);
    for(let i = 0; i < Math.max(pair1.length, pair2.length); i++) {
        let p1 = (pair1[i] == undefined) ? null : pair1[i]; 
        let p2 = (pair2[i] == undefined) ? null : pair2[i];
        console.log(" ".repeat(depth) + "p1: ", p1);
        console.log(" ".repeat(depth) + "p2: ", p2);
        if (p1 != null && p2 == null) { //p2 outlasts p1
            return -1;
        } else if (p1 == null && p2 != null) {
            return 1;
        } else if (Array.isArray(p1) || Array.isArray(p2)) {
            p1 = (Array.isArray(p1)) ? p1 : [p1];
            p2 = (Array.isArray(p2)) ? p2 : [p2];
            let c = compare(p1, p2, depth+1);
            if (c != 0) {
                return c;
            }
        } else { //comparing numbers
            if (p1 > p2) {
                return -1;
            } else if (p1 < p2){
                return 1;
            }
        }
    }
    return 0;
}



// partA();
partB();