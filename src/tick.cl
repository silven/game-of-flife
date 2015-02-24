
int wrap(const int n, const int size) {
	if (n < 0) {
		return size + n;
	} else if (n >= size) {
		return n - size;
	} else {
		return n;
	}
}

__kernel void tick(__global const int *current,
	               __global int *next) {

	int x = get_global_id(0);
	int y = get_global_id(1);

	int SIZE = get_global_size(0);
	int idx = y * SIZE + x;

	int state = current[idx];
	int neighbors = 0;
	for(int dx = -1; dx < 2; dx++) {
		for(int dy = -1; dy < 2; dy++) {
			if (dx == 0 && dy == 0) {
				continue;
			}

			int nx = wrap(x + dx, SIZE);
			int ny = wrap(y + dy, SIZE);

			int ndx = ny * SIZE + nx;
			neighbors += current[ndx];
		}
	}

	int next_state = 0;
	if (state == 1 && neighbors == 2) {
		next_state = 1;
	} else if (neighbors == 3) {
		next_state = 1;
	} else {
		next_state = 0;
	}
	next[idx] = next_state;
}

