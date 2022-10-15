type ConstructForm = {
  cells: number[];
};

export class Block {
  static fromCells(cells: number[]) {
    return new Block({cells});
  }

  private cells: number[];
  private width: number;
  private height: number;

  private constructor(form: ConstructForm) {
    const {cells} = form;
    this.cells = cells;
    this.width = 0;
    this.height = 0;
    for (const cell of cells) {
      let w = (cell % 9) + 1;
      let h = Math.floor(cell / 9) + 1;
      this.width = w > this.width ? w : this.width;
      this.height = h > this.height ? h : this.height;
    }
  }

  getWidth(): number {
    return this.width || 0;
  }

  getHeight(): number {
    return this.height || 0;
  }

  getCells(): number[] {
    return this.cells || [];
  }
}
