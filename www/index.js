import {greet} from '../pkg';
import("../pkg/index.js").catch(console.error);

greet("World")