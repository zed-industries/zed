// class template regex -*- C++ -*-

// Copyright (C) 2013-2020 Free Software Foundation, Inc.
//
// This file is part of the GNU ISO C++ Library.  This library is free
// software; you can redistribute it and/or modify it under the
// terms of the GNU General Public License as published by the
// Free Software Foundation; either version 3, or (at your option)
// any later version.

// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// Under Section 7 of GPL version 3, you are granted additional
// permissions described in the GCC Runtime Library Exception, version
// 3.1, as published by the Free Software Foundation.

// You should have received a copy of the GNU General Public License and
// a copy of the GCC Runtime Library Exception along with this program;
// see the files COPYING3 and COPYING.RUNTIME respectively.  If not, see
// <http://www.gnu.org/licenses/>.

/**
 *  @file bits/regex_executor.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{regex}
 */

// FIXME convert comments to doxygen format.

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

namespace __detail
{
  /**
   * @addtogroup regex-detail
   * @{
   */

  /**
   * @brief Takes a regex and an input string and does the matching.
   *
   * The %_Executor class has two modes: DFS mode and BFS mode, controlled
   * by the template parameter %__dfs_mode.
   */
  template<typename _BiIter, typename _Alloc, typename _TraitsT,
	   bool __dfs_mode>
    class _Executor
    {
      using __search_mode = integral_constant<bool, __dfs_mode>;
      using __dfs = true_type;
      using __bfs = false_type;

      enum class _Match_mode : unsigned char { _Exact, _Prefix };

    public:
      typedef typename iterator_traits<_BiIter>::value_type _CharT;
      typedef basic_regex<_CharT, _TraitsT>                 _RegexT;
      typedef std::vector<sub_match<_BiIter>, _Alloc>       _ResultsVec;
      typedef regex_constants::match_flag_type              _FlagT;
      typedef typename _TraitsT::char_class_type            _ClassT;
      typedef _NFA<_TraitsT>                                _NFAT;

    public:
      _Executor(_BiIter         __begin,
		_BiIter         __end,
		_ResultsVec&    __results,
		const _RegexT&  __re,
		_FlagT          __flags)
      : _M_begin(__begin),
      _M_end(__end),
      _M_re(__re),
      _M_nfa(*__re._M_automaton),
      _M_results(__results),
      _M_rep_count(_M_nfa.size()),
      _M_states(_M_nfa._M_start(), _M_nfa.size()),
      _M_flags((__flags & regex_constants::match_prev_avail)
	       ? (__flags
		  & ~regex_constants::match_not_bol
		  & ~regex_constants::match_not_bow)
	       : __flags)
      { }

      // Set matched when string exactly matches the pattern.
      bool
      _M_match()
      {
	_M_current = _M_begin;
	return _M_main(_Match_mode::_Exact);
      }

      // Set matched when some prefix of the string matches the pattern.
      bool
      _M_search_from_first()
      {
	_M_current = _M_begin;
	return _M_main(_Match_mode::_Prefix);
      }

      bool
      _M_search();

    private:
      void
      _M_rep_once_more(_Match_mode __match_mode, _StateIdT);

      void
      _M_handle_repeat(_Match_mode, _StateIdT);

      void
      _M_handle_subexpr_begin(_Match_mode, _StateIdT);

      void
      _M_handle_subexpr_end(_Match_mode, _StateIdT);

      void
      _M_handle_line_begin_assertion(_Match_mode, _StateIdT);

      void
      _M_handle_line_end_assertion(_Match_mode, _StateIdT);

      void
      _M_handle_word_boundary(_Match_mode, _StateIdT);

      void
      _M_handle_subexpr_lookahead(_Match_mode, _StateIdT);

      void
      _M_handle_match(_Match_mode, _StateIdT);

      void
      _M_handle_backref(_Match_mode, _StateIdT);

      void
      _M_handle_accept(_Match_mode, _StateIdT);

      void
      _M_handle_alternative(_Match_mode, _StateIdT);

      void
      _M_dfs(_Match_mode __match_mode, _StateIdT __start);

      bool
      _M_main(_Match_mode __match_mode)
      { return _M_main_dispatch(__match_mode, __search_mode{}); }

      bool
      _M_main_dispatch(_Match_mode __match_mode, __dfs);

      bool
      _M_main_dispatch(_Match_mode __match_mode, __bfs);

      bool
      _M_is_word(_CharT __ch) const
      {
	static const _CharT __s[2] = { 'w' };
	return _M_re._M_automaton->_M_traits.isctype
	  (__ch, _M_re._M_automaton->_M_traits.lookup_classname(__s, __s+1));
      }

      bool
      _M_at_begin() const
      {
	return _M_current == _M_begin
	  && !(_M_flags & (regex_constants::match_not_bol
			   | regex_constants::match_prev_avail));
      }

      bool
      _M_at_end() const
      {
	return _M_current == _M_end
	  && !(_M_flags & regex_constants::match_not_eol);
      }

      bool
      _M_word_boundary() const;

      bool
      _M_lookahead(_StateIdT __next);

       // Holds additional information used in BFS-mode.
      template<typename _SearchMode, typename _ResultsVec>
	struct _State_info;

      template<typename _ResultsVec>
	struct _State_info<__bfs, _ResultsVec>
	{
	  explicit
	  _State_info(_StateIdT __start, size_t __n)
	  : _M_visited_states(new bool[__n]()), _M_start(__start)
	  { }

	  bool _M_visited(_StateIdT __i)
	  {
	    if (_M_visited_states[__i])
	      return true;
	    _M_visited_states[__i] = true;
	    return false;
	  }

	  void _M_queue(_StateIdT __i, const _ResultsVec& __res)
	  { _M_match_queue.emplace_back(__i, __res); }

	  // Dummy implementations for BFS mode.
	  _BiIter* _M_get_sol_pos() { return nullptr; }

	  // Saves states that need to be considered for the next character.
	  vector<pair<_StateIdT, _ResultsVec>>	_M_match_queue;
	  // Indicates which states are already visited.
	  unique_ptr<bool[]>			_M_visited_states;
	  // To record current solution.
	  _StateIdT _M_start;
	};

      template<typename _ResultsVec>
	struct _State_info<__dfs, _ResultsVec>
	{
	  explicit
	  _State_info(_StateIdT __start, size_t) : _M_start(__start)
	  { }

	  // Dummy implementations for DFS mode.
	  bool _M_visited(_StateIdT) const { return false; }
	  void _M_queue(_StateIdT, const _ResultsVec&) { }

	  _BiIter* _M_get_sol_pos() { return &_M_sol_pos; }

	  // To record current solution.
	  _StateIdT _M_start;
	  _BiIter   _M_sol_pos;
	};

    public:
      _ResultsVec                                           _M_cur_results;
      _BiIter                                               _M_current;
      _BiIter                                               _M_begin;
      const _BiIter                                         _M_end;
      const _RegexT&                                        _M_re;
      const _NFAT&                                          _M_nfa;
      _ResultsVec&                                          _M_results;
      vector<pair<_BiIter, int>>                            _M_rep_count;
      _State_info<__search_mode, _ResultsVec>		    _M_states;
      _FlagT                                                _M_flags;
      // Do we have a solution so far?
      bool                                                  _M_has_sol;
    };

 //@} regex-detail
} // namespace __detail
_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

#include <bits/regex_executor.tcc>
