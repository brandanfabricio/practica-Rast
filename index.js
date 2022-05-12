const ffi = require('ffi');

const lib = ffi.Library('./tuto-pdf/embeber/target/release/libembeber',{
    'procesar': ['void',[]]
});
lib.procesar();

console.log('completado!');