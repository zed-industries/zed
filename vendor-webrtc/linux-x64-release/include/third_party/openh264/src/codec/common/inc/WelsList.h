/*!
 * \copy
 *     Copyright (c)  2009-2015, Cisco Systems
 *     All rights reserved.
 *
 *     Redistribution and use in source and binary forms, with or without
 *     modification, are permitted provided that the following conditions
 *     are met:
 *
 *        * Redistributions of source code must retain the above copyright
 *          notice, this list of conditions and the following disclaimer.
 *
 *        * Redistributions in binary form must reproduce the above copyright
 *          notice, this list of conditions and the following disclaimer in
 *          the documentation and/or other materials provided with the
 *          distribution.
 *
 *     THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 *     "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 *     LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 *     FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 *     COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
 *     INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
 *     BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 *     LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 *     CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 *     LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN
 *     ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 *     POSSIBILITY OF SUCH DAMAGE.
 *
 *
 * \file    WelsList
 *
 * \brief   for the list function needed in ThreadPool
 *
 * \date    9/27/2015 Created
 *
 *************************************************************************************
 */


#ifndef _WELS_LIST_H_
#define _WELS_LIST_H_

#include "typedefs.h"
#include <stdlib.h>

namespace WelsCommon {

template<typename TNodeType>
struct SNode {
  TNodeType* pPointer;
  SNode* pPrevNode;
  SNode* pNextNode;
};

template<typename TNodeType>
class CWelsList {
 public:
  CWelsList() {
    m_iCurrentNodeCount = 0;
    m_iMaxNodeCount = 50;

    m_pCurrentList = NULL;
    m_pFirst = NULL;
    m_pCurrent = NULL;
    m_pLast = NULL;
  };
  ~CWelsList() {
    if (m_pCurrentList) {
      free (m_pCurrentList);
      m_pCurrentList = NULL;
    }

    m_pCurrentList = NULL;
    m_pFirst = NULL;
    m_pCurrent = NULL;
    m_pLast = NULL;
  };

  int32_t size() {
    return m_iCurrentNodeCount;
  }

  bool push_back (TNodeType* pNode) {
    if (!pNode) {
      return false;
    }

    if (NULL == m_pCurrentList) {
      m_pCurrentList = static_cast<SNode<TNodeType>*> (malloc (m_iMaxNodeCount * sizeof (SNode<TNodeType>)));
      if (NULL == m_pCurrentList) {
        return false;
      } else {
        ResetStorage();
      }
    }

    if (NULL == m_pCurrent) {
      if (!ExpandList()) {
        return false;
      }
    }

    m_pCurrent->pPointer = pNode;
    m_pCurrent = m_pCurrent->pNextNode;
    m_iCurrentNodeCount++;

    return true;
  }

  TNodeType* begin() {
    if (m_pFirst) {
      return m_pFirst->pPointer;
    }
    return NULL;
  }

  void pop_front() {
    if (m_iCurrentNodeCount == 0) {
      return;
    }

    SNode<TNodeType>* pTemp = m_pFirst;

    m_pFirst = m_pFirst->pNextNode;
    m_pFirst->pPrevNode = NULL;

    CleanOneNode (pTemp);

    m_pLast->pNextNode = pTemp;
    pTemp->pPrevNode = m_pLast;
    m_pLast = pTemp;

    if (NULL == m_pCurrent)
      m_pCurrent = m_pLast;

    m_iCurrentNodeCount --;
  }

  bool erase (TNodeType* pNode) {
    if (0 == m_iCurrentNodeCount) {
      return false;
    }

    SNode<TNodeType>* pTemp = m_pFirst;
    do {
      if (pNode == pTemp->pPointer) {
        if (pTemp->pPrevNode) {
          pTemp->pPrevNode->pNextNode = pTemp->pNextNode;
        } else {
          m_pFirst = pTemp->pNextNode;
        }

        if (pTemp->pNextNode) {
          pTemp->pNextNode->pPrevNode = pTemp->pPrevNode;
        }

        CleanOneNode (pTemp);
        m_iCurrentNodeCount --;

        m_pLast->pNextNode = pTemp;
        pTemp->pPrevNode = m_pLast;
        m_pLast = pTemp;

        return true;
      }

      pTemp = pTemp->pNextNode;

    } while (pTemp && pTemp->pPointer);
    return false;
  }

  bool findNode (TNodeType* pNodeTarget) {
    if ((m_iCurrentNodeCount > 0) && pNodeTarget) {
      SNode<TNodeType>* pNode = m_pFirst;
      while (pNode) {
        if (pNode->pPointer == pNodeTarget) {
          return true;
        }
        pNode = pNode->pNextNode;
      }
    }
    return false;
  }

  TNodeType* getNode (int iNodeIdx) {
    if ((iNodeIdx > m_iCurrentNodeCount - 1) || (0 == m_iCurrentNodeCount)) {
      return NULL;
    }
    SNode<TNodeType>* pNode = m_pFirst;
    for (int i = 0; i < iNodeIdx; i++) {
      if (pNode->pNextNode) {
        pNode = pNode->pNextNode;
      } else {
        return NULL;
      }
    }
    return pNode->pPointer;
  }

 private:
  bool ExpandList() {
    SNode<TNodeType>* tmpCurrentList = static_cast<SNode<TNodeType>*> (malloc (m_iMaxNodeCount * 2 * sizeof (
                                         SNode<TNodeType>)));
    if (tmpCurrentList == NULL) {
      return false;
    }
    InitStorage (tmpCurrentList, (m_iMaxNodeCount * 2) - 1);

    SNode<TNodeType>* pTemp = m_pFirst;
    for (int i = 0; ((i < m_iMaxNodeCount) && pTemp); i++) {
      tmpCurrentList[i].pPointer = pTemp->pPointer;
      pTemp = pTemp->pNextNode;
    }

    free (m_pCurrentList);
    m_pCurrentList = tmpCurrentList;
    m_iCurrentNodeCount = m_iMaxNodeCount;
    m_iMaxNodeCount = m_iMaxNodeCount * 2;
    m_pFirst = & (m_pCurrentList[0]);
    m_pLast = & (m_pCurrentList[m_iMaxNodeCount - 1]);
    m_pCurrent = & (m_pCurrentList[m_iCurrentNodeCount]);
    return true;
  }

  void InitStorage (SNode<TNodeType>* pList, const int32_t iMaxIndex) {
    pList[0].pPrevNode = NULL;
    pList[0].pPointer = NULL;
    pList[0].pNextNode = & (pList[1]);
    for (int i = 1; i < iMaxIndex; i++) {
      pList[i].pPrevNode = & (pList[i - 1]);
      pList[i].pPointer = NULL;
      pList[i].pNextNode = & (pList[i + 1]);
    }
    pList[iMaxIndex].pPrevNode = & (pList[iMaxIndex - 1]);
    pList[iMaxIndex].pPointer = NULL;
    pList[iMaxIndex].pNextNode = NULL;
  }


  void CleanOneNode (SNode<TNodeType>* pSNode) {
    pSNode->pPointer = NULL;
    pSNode->pPrevNode = NULL;
    pSNode->pNextNode = NULL;
  }

  void ResetStorage() {
    InitStorage (m_pCurrentList, m_iMaxNodeCount - 1);
    m_pCurrent = m_pCurrentList;
    m_pFirst = & (m_pCurrentList[0]);
    m_pLast = & (m_pCurrentList[m_iMaxNodeCount - 1]);
  }

 private:
  int32_t m_iCurrentNodeCount;
  int32_t m_iMaxNodeCount;
  SNode<TNodeType>* m_pCurrentList;
  SNode<TNodeType>* m_pFirst;
  SNode<TNodeType>* m_pLast;
  SNode<TNodeType>* m_pCurrent;
};

template<typename TNodeType>
class CWelsNonDuplicatedList : public CWelsList<TNodeType> {
 public:
  bool push_back (TNodeType* pNode) {
    if (0 != this->size()) {
      if ((NULL != pNode) && (this->findNode (pNode))) {      //not checking NULL for easier testing
        return false;
      }
    }

    return CWelsList<TNodeType>::push_back (pNode);
  }

};


}


#endif



