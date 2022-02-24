import { ParserPointer } from '../utils/ParserPointer';
import { SyntaxError } from '../errors/SyntaxError';

import { Others } from './Others';

export class String {
	private value = '';

	public others: Others;

	constructor(private pointer: ParserPointer) {
		this.others = new Others(pointer);
	}

	string() {
		const { pointer, others } = this;

		const token = pointer.token;
		if (!token || !['DoubleQuote', 'SingleQuote'].includes(token.type)) return null;

		const delimiter = token;
		const startLine = pointer.line;

		pointer.take(delimiter.type);

		for (;;) {
			if (!pointer.token || ['EndFile', 'NewLine'].includes(pointer.token.type))
				new SyntaxError(pointer, {
					startLine,
					lineError: pointer.line - 1,
					reason: `Expected a '${delimiter.value}'`,
					isParser: true,
				});

			const expr = others.others();
			if (expr && expr.type === 'SpaceCaracter') {
				this.value += expr.value;

				continue;
			}

			if (pointer.token?.type === delimiter.type) break;

			pointer.next(true, true, false);
		}

		pointer.take(delimiter.type);

		return {
			type: 'String',
			value: this.value,
		};
	}
}