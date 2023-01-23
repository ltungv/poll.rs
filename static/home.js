function handleClickBallotRegisterButton() {
	const uuidInput = document.getElementById('ballot-uuid-input');
	uuidInput.value = crypto.randomUUID();
}
