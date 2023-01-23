let sortable_item_ids = Sortable.create(
	document.getElementById('ballot-rankings-container'), {
		onEnd: evt => {
			let sorted_item_ids = sortable_item_ids.toArray();
			let ranked_item_ids = sorted_item_ids
				.slice(0, sorted_item_ids.indexOf('delimiter'))
				.map(n => parseInt(n, 10));

			fetch( "/ballot", {
				method: 'POST', 
				headers: {
				  'Content-Type': 'application/json'
				},
				redirect: 'follow',
				body: JSON.stringify({ ranked_item_ids })
			});
		},
	});
