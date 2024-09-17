// s0::e1_e2_e3
// s1::e1_e3_e2
// s2::e2_e1_e3
// s3::e2_e3_e1
// s4::e3_e1_e2
// s5::e3_e2_e1

package pkg_Arbiter3;

  // width: log2(fact(elem_cnt))
  // enum label encodes as 'least recently used' to 'most'
  typedef enum bit [2:0] {
    e1_e2_e3,
    e1_e3_e2,
    e2_e1_e3,
    e2_e3_e1,
    e3_e1_e2,
    e3_e2_e1
  } state_t;

endpackage: pkg_Arbiter3

module Arbiter3 
       import pkg_Arbiter::*; 
       #(
       ) 
       (
          input bit reset,  // active high
          input bit clock,
          input logic [2:0] req,  // width: elem_cnt
          output state_t gnt 
       );

  state_t cur_state, nxt_state;

  // TODO: add valid/mask

  // TODO: assert 'in' not X/Z
  // TODO: cover states, transitions

  always_ff @(posedge clock) begin
    unique casez ({reset, cur_state, req})
      {1'b1, ___, ___}, // synchronous reset
      {1'b0, e1_e2_e3, 3'b1__},
      {1'b0, e2_e1_e3, 3'b01_},
      {1'b0, e2_e3_e1, 3'b001}: nxt_state <= e2_e3_e1;


    endcase
  end

endmodule: Arbiter3