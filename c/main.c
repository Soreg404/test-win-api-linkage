int MessageBoxA(void*, char*, char*, unsigned);

int main() {
	MessageBoxA(
		(void*)0, // NULL
		"content",
		"title",
		0 // MB_OK
	);
}
