# ARQUITECTURA DE LA BLOCKCHAIN

###1- Bloques:
Cada bloque contendrá:
- 'index': Posición del bloque en la cadena.
- 'timestamp': Marca de tiempo de creación.
- 'data': Info dentro del bloque (p.e. transacciones).
- 'previous_hash': Hash del bloque anterior.
- 'hash': (Calculado mediante , p.e., SHA-256)
- 'difficulty': cantidad de bytes de dificultad para cumplir con el PoW. (Para este caso concreto, lo mantendremos igual para todos los bloques).

###2- Cadena de bloques:
- Una lista encadenada de bloques.
- Debe mantener la integridad referenciando hashes de bloques previos.

###3- Consenso:
Para empezar, usaremos un algoritmo sencillo:
- Proof of work (PoW): los nodos deben resolver un problema computacional (p.e. encontrar un hash con un número de ceros concretos).
- Alternativa: modelo simple de "autoridad" para agregar bloques sin pruebas complejas.

###4- Nodos y red:
- Define nodos que mantengan una copia de la blockchain y propaguen bloques.
- Usa un protocolo simple (HTTP o WebSocket) para la comunicación entre nodos.

# TECNOLOGÍAS NECESARIAS

- SHA-2: Para hash functions.
- Serde: Serializar y deserializar datos.
- Tokio o Warp: Para manejar conexiones HTTP o WebSocket (opcional si red elijo red p2p).

# DISEÑO DEL PROYECTO

- Block: Define estructura de los bloques y funciones "extra".
- Blockchain: Maneja la lista encadenada de bloques y lógica para agregar nuevos.
- Mining: Implementa PoW (o el algoritmo de consenso).
- Networking: (Opcional al inicio) Implementa nodos y comunicación.
