// 简易递归操作中悟出道理
const recursion = () => {
    return recursion
}
// 这里能看出必须，输出=~= 输入， 并且带一下参数，来区分异同

const recursion_with_params = (params:number) => {
    // 并且有出口
    if (params == 1) {
        return 100;
    }
    return recursion_with_params(--params);
}

console.log(recursion_with_params(3)); 
