const cp = require('child_process');
console.log(cp.execSync('git -C ./knotted-portal-master show HEAD:src/shaders/vertex.glsl').toString());
