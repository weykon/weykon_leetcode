// 简易递归操作中悟出道理
var recursion = function () {
    return recursion;
};
// 这里能看出必须，输出=~= 输入， 并且带一下参数，来区分异同
var recursion_with_params = function (params) {
    // 并且有出口
    if (params == 1) {
        return 100;
    }
    return recursion_with_params(--params);
};
console.log(recursion_with_params(3));
